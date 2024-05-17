// フロントのみでの動作確認の際などに再び使うかもしれないので，ダミーデータの残骸を置いておく

import { AcAsmPostReq, AcAssembleNonUlid } from "../../../share/assemble_type";

// const acData: AssembleCardProps[] = [
//   {
//     uuid: "test1",
//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test2",
//     acPilotName: "Jane Smith",
//     acName: "Iron Guardian",
//     acImageUrl: "/ac/iron.jpg",
//     emblemImageUrl: "/ac/steel.jpg",
//   },
//   {
//     uuid: "test3",
//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test4",
//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",
//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",

//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",

//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",

//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",

//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",

//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",

//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   {
//     uuid: "test1",

//     acPilotName: "John Doe",
//     acName: "Armored Steel Borne",
//     acImageUrl: "/ac/ac.jpg",
//     emblemImageUrl: "/ac/rusty.jpg",
//   },
//   // Add more AC data objects as needed
// ];


// const defaultAcAssemble: AcAssemble = {
//   ulid: "",
//   pilotName: "Jhon Doe",
//   acName: "",
//   acCardImageUrl: "",
//   emblemImageUrl: "",
//   acImageUrls: [""],
//   parts: {
//     weapons: {
//       rArm: "Rifle",
//       lArm: "Shield",
//       rBack: "Missile",
//       lBack: "Rifle",
//     },
//     frame: {
//       head: "Head",
//       core: "Core",
//       arms: "Arms",
//       legs: "Legs",
//     },
//   },
//   description: "",
//   remarks: "",
// };

const dummyAcAssembleNonUlid: AcAssembleNonUlid = {
    pilotName: "Dummy Pilot",
    acName: "Dummy AC",
    acCardImageUrl: "http://example.com/dummy.jpg",
    emblemImageUrl: "http://example.com/emblem.jpg",
    acImageUrls: ["http://example.com/ac1.jpg", "http://example.com/ac2.jpg"],
    parts: {
        weapons: 
        {
            rArm: "",
            lArm: "",
            rBack: "",
            lBack: ""
        },
        frame: {
            head: "",
            core: "",
            arms: "",
            legs: ""
        }
    },
    description: "This is a dummy AC.",
    remarks: "No remarks."
};

export const dummyAcAsmPostReq: AcAsmPostReq = {
    acAssemble: dummyAcAssembleNonUlid
};