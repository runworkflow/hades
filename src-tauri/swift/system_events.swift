import AppKit
import Foundation

private let eventSleep: Int32 = 1
private let eventScreenLock: Int32 = 2
private let eventUserSwitch: Int32 = 3

private var observers: [NSObjectProtocol] = []

@_cdecl("hades_system_events_start")
public func hadesSystemEventsStart(callback: @escaping @convention(c) (Int32) -> Void) {
    guard observers.isEmpty else { return }

    let workspaceCenter = NSWorkspace.shared.notificationCenter
    let distributedCenter = DistributedNotificationCenter.default()

    observers.append(
        workspaceCenter.addObserver(forName: NSWorkspace.willSleepNotification, object: nil, queue: nil) { _ in
            callback(eventSleep)
        })
    observers.append(
        workspaceCenter.addObserver(forName: NSWorkspace.sessionDidResignActiveNotification, object: nil, queue: nil) { _ in
            callback(eventUserSwitch)
        })
    observers.append(
        distributedCenter.addObserver(forName: Notification.Name("com.apple.screenIsLocked"), object: nil, queue: nil) { _ in
            callback(eventScreenLock)
        })
}
