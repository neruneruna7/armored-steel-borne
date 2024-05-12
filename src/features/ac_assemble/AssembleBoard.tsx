import Image from "next/image";
import Link from "next/link";
import { AcAssemble } from "../../../share/assemble_type";


interface AssembleCardProps {
  uuid: string, // UUIDもしくはULIDで一意なIDを付与
  acPilotName: string,
  acName: string;
  acImageUrl: string;
  emblemImageUrl: string;
}

function AssembleCard({ ac }: { ac: AssembleCardProps }) {
  return (
    // いい感じのホバー時のスタイルがわからん
    <Link href={{
      pathname: "/debug",
      query: { uuid: ac.uuid }
    }} className="hover:mix-blend-luminosity hover:bg-gray-600" >
      <div className="border-4 w-80 h-72 relative">
        <div className="flex h-auto border-2">
          <div className="">
            <Image src={ac.emblemImageUrl} alt={`Mech Image`} width={70} height={70} className="object-contain border-2" />
          </div>
          <div className="ml-4">
            <h2>PILOT: {ac.acPilotName}</h2>
            <h2>AC: {ac.acName}</h2>
          </div>
          {/* <div className="border-4">
          <p>{mech.description}</p>
        </div> */}
        </div>
        <div className="border-2 relative w-full h-56">
          <Image src={ac.acImageUrl} alt={`Mech Image`} fill className="object-contain" />
        </div>
      </div>
    </Link>
  );
}


const acData: AssembleCardProps[] = [
  {
    uuid: "test1",
    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test2",
    acPilotName: "Jane Smith",
    acName: "Iron Guardian",
    acImageUrl: "/ac/iron.jpg",
    emblemImageUrl: "/ac/steel.jpg",
  },
  {
    uuid: "test3",
    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test4",
    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",
    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",

    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",

    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",

    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",

    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",

    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",

    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  {
    uuid: "test1",

    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
  },
  // Add more AC data objects as needed
];


export default function AssembleBoard() {

  const acCards = acData.map((acAsm, index) => (
    <AssembleCard key={index} ac={acAsm} />
  ));

  return (
    <div className="min-h-full w-full flex flex-col justify-center items-center gap-10">
      <h1>AC Assemble</h1>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        {acCards}
      </div>
    </div>
  );
}