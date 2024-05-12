use share::model::Assemble::AcAssemble;
use ulid::Ulid;

pub(crate) struct AcAsmGetReq {
    ulid: Ulid
}

pub(crate) struct AcAsmGetRes {
    ac_assemble: AcAssemble
}