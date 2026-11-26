import CryptoKit
import Foundation
import LocalAuthentication
import Security

private let statusOk: Int32 = 0
private let statusCancelled: Int32 = 1
private let statusUnavailable: Int32 = 2
private let statusFailed: Int32 = 4

private let derivedKeyLength = 32
private let keyDerivationInfo = Data("hades:biometric:v1".utf8)

private func askingContext(_ reason: UnsafePointer<CChar>) -> LAContext {
    let context = LAContext()
    context.localizedReason = String(cString: reason)
    return context
}

private func status(for error: Error) -> Int32 {
    guard let authenticationError = error as? LAError else { return statusFailed }
    switch authenticationError.code {
    case .userCancel, .systemCancel, .appCancel:
        return statusCancelled
    case .biometryNotAvailable, .biometryNotEnrolled:
        return statusUnavailable
    default:
        return statusFailed
    }
}

private func derivedKeyBytes(from secret: SharedSecret) -> SymmetricKey {
    secret.hkdfDerivedSymmetricKey(
        using: SHA256.self,
        salt: Data(),
        sharedInfo: keyDerivationInfo,
        outputByteCount: derivedKeyLength
    )
}

private func copy(_ key: SymmetricKey, to destination: UnsafeMutablePointer<UInt8>) {
    key.withUnsafeBytes { bytes in
        bytes.copyBytes(to: UnsafeMutableRawBufferPointer(start: destination, count: derivedKeyLength))
    }
}

@_cdecl("hades_biometrics_available")
public func hadesBiometricsAvailable() -> Int32 {
    guard SecureEnclave.isAvailable else { return 0 }
    return LAContext().canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: nil) ? 1 : 0
}

@_cdecl("hades_biometrics_enroll")
public func hadesBiometricsEnroll(
    reason: UnsafePointer<CChar>,
    blobOut: UnsafeMutablePointer<UInt8>,
    blobCapacity: Int,
    blobLength: UnsafeMutablePointer<Int>,
    publicKeyOut: UnsafeMutablePointer<UInt8>,
    publicKeyCapacity: Int,
    keyOut: UnsafeMutablePointer<UInt8>
) -> Int32 {
    guard hadesBiometricsAvailable() == 1 else { return statusUnavailable }
    do {
        let access = SecAccessControlCreateWithFlags(
            nil, kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
            [.privateKeyUsage, .biometryCurrentSet], nil)!
        let created = try SecureEnclave.P256.KeyAgreement.PrivateKey(accessControl: access)
        let blob = created.dataRepresentation

        let ephemeral = P256.KeyAgreement.PrivateKey()
        let publicKey = ephemeral.publicKey.x963Representation
        guard blob.count <= blobCapacity, publicKey.count <= publicKeyCapacity else { return statusFailed }

        let reloaded = try SecureEnclave.P256.KeyAgreement.PrivateKey(
            dataRepresentation: blob, authenticationContext: askingContext(reason))
        let secret = try reloaded.sharedSecretFromKeyAgreement(with: ephemeral.publicKey)

        blob.copyBytes(to: blobOut, count: blob.count)
        blobLength.pointee = blob.count
        publicKey.copyBytes(to: publicKeyOut, count: publicKey.count)
        copy(derivedKeyBytes(from: secret), to: keyOut)
        return statusOk
    } catch {
        return status(for: error)
    }
}

@_cdecl("hades_biometrics_derive")
public func hadesBiometricsDerive(
    reason: UnsafePointer<CChar>,
    blob: UnsafePointer<UInt8>,
    blobLength: Int,
    publicKey: UnsafePointer<UInt8>,
    publicKeyLength: Int,
    keyOut: UnsafeMutablePointer<UInt8>
) -> Int32 {
    guard hadesBiometricsAvailable() == 1 else { return statusUnavailable }
    do {
        let key = try SecureEnclave.P256.KeyAgreement.PrivateKey(
            dataRepresentation: Data(bytes: blob, count: blobLength),
            authenticationContext: askingContext(reason))
        let partner = try P256.KeyAgreement.PublicKey(
            x963Representation: Data(bytes: publicKey, count: publicKeyLength))
        let secret = try key.sharedSecretFromKeyAgreement(with: partner)
        copy(derivedKeyBytes(from: secret), to: keyOut)
        return statusOk
    } catch {
        return status(for: error)
    }
}
