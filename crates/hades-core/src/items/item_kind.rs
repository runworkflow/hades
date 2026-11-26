use serde::{Deserialize, Serialize};

pub struct FieldSpec {
    pub key: &'static str,
    pub is_secret: bool,
}

const PASSWORD_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        key: "username",
        is_secret: false,
    },
    FieldSpec {
        key: "password",
        is_secret: true,
    },
    FieldSpec {
        key: "website",
        is_secret: false,
    },
];

const API_KEY_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        key: "service",
        is_secret: false,
    },
    FieldSpec {
        key: "key",
        is_secret: true,
    },
];

const SSH_KEY_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        key: "host",
        is_secret: false,
    },
    FieldSpec {
        key: "privateKey",
        is_secret: true,
    },
];

const NOTE_FIELDS: &[FieldSpec] = &[FieldSpec {
    key: "text",
    is_secret: true,
}];

const FILE_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        key: "fileName",
        is_secret: false,
    },
    FieldSpec {
        key: "size",
        is_secret: false,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemKind {
    Password,
    ApiKey,
    SshKey,
    Note,
    File,
}

impl ItemKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Password => "password",
            Self::ApiKey => "apiKey",
            Self::SshKey => "sshKey",
            Self::Note => "note",
            Self::File => "file",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        [
            Self::Password,
            Self::ApiKey,
            Self::SshKey,
            Self::Note,
            Self::File,
        ]
        .into_iter()
        .find(|kind| kind.as_str() == text)
    }

    pub fn fields(self) -> &'static [FieldSpec] {
        match self {
            Self::Password => PASSWORD_FIELDS,
            Self::ApiKey => API_KEY_FIELDS,
            Self::SshKey => SSH_KEY_FIELDS,
            Self::Note => NOTE_FIELDS,
            Self::File => FILE_FIELDS,
        }
    }

    pub fn field(self, key: &str) -> Option<&'static FieldSpec> {
        self.fields().iter().find(|spec| spec.key == key)
    }
}
