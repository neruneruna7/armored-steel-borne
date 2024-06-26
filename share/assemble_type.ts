/*
 Generated by typeshare 1.9.2
*/

export type AcAsmGetReq = string;

export interface Weapons {
	rArm: string;
	lArm: string;
	rBack: string;
	lBack: string;
}

export interface Frame {
	head: string;
	core: string;
	arms: string;
	legs: string;
}

export interface Parts {
	weapons: Weapons;
	frame: Frame;
}

export interface AcAssemble {
	ulid: string;
	pilotName: string;
	acName: string;
	acCardImageUrl: string;
	emblemImageUrl: string;
	acImageUrls: string[];
	parts: Parts;
	description: string;
	remarks: string;
}

export interface AcAssembleNonUlid {
	pilotName: string;
	acName: string;
	acCardImageUrl: string;
	emblemImageUrl: string;
	acImageUrls: string[];
	parts: Parts;
	description: string;
	remarks: string;
}

export interface AcAsmGetRes {
	acAssemble: AcAssemble;
}

export interface AcAsmListReq {
	prevId?: string;
	size?: number;
}

export interface AcAsmListRes {
	acAssembles: AcAssemble[];
}

export interface AcAsmUpdateReq {
	acAssemble: AcAssemble;
}

export interface AcAsmPostReq {
	acAssemble: AcAssembleNonUlid;
}

