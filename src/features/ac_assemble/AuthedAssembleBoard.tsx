import Image from "next/image";
import Link from "next/link";
import { AcAsmListRes, AcAssemble } from "../../../share/assemble_type";

interface AssembleCardProps {
  id: number,
  pilotName: string,
  acName: string;
  acCardImageUrl: string;
  emblemImageUrl: string;
}

function AuthedAssembleCard({ ac }: { ac: AssembleCardProps }) {

  return (
    // いい感じのホバー時のスタイルがわからん
    <Link href={{
      pathname: `/asmdashboard/${ac.id}`,
    }} className="hover:mix-blend-luminosity hover:bg-gray-600" >
      <div className="border-4 w-80 h-72 relative">
        <div className="flex h-auto border-2">
          <div className="">
            <Image src={ac.emblemImageUrl} alt={`Mech Image`} width={70} height={70} className="object-contain border-2" />
          </div>
          <div className="ml-4">
            <h2>PILOT: {ac.pilotName}</h2>
            <h2>AC: {ac.acName}</h2>
          </div>
        </div>
        <div className="border-2 relative w-full h-56">
          <Image src={ac.acCardImageUrl} alt={`Mech Image`} fill className="object-contain" />
        </div>
      </div>
    </Link>
  );
}

function convertAcAssembleToAcAsmCardProps(acAssemble: AcAssemble): AssembleCardProps {
  // ここでacAssembleをAcAsmCardPropsに変換します。
  const re: AssembleCardProps = {
    id: acAssemble.id,
    pilotName: acAssemble.pilotName,
    acName: acAssemble.acName,
    acCardImageUrl: acAssemble.acCardImageUrl,
    emblemImageUrl: acAssemble.emblemImageUrl,
  };
  return re;
}



const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
async function listAsm(): Promise<AcAsmListRes> {
  const ASSEMBLE_LIST_URL = "http://127.0.0.1:8000/api/asm/list";

  const res = await fetch(`${ASSEMBLE_LIST_URL}`);
  console.log(res);
  const data = await res.json();
  console.log(data);
  return data;
}

let acAsmList: AcAsmListRes;
export default function AssembleBoard() {
  console.log("AuthedAssembleBoard")
  if (!acAsmList) {
    throw listAsm().then((data) => (acAsmList = data));
  }

  const asmCardProps = acAsmList.acAssembles.map((acAsm) => convertAcAssembleToAcAsmCardProps(acAsm));

  const acCards = asmCardProps.map((acAsm, index) => (
    <AuthedAssembleCard key={index} ac={acAsm} />
  ));

  return (
    <div className="min-h-full w-full flex flex-col justify-center items-center gap-10">
      <h1>Authed</h1>
      <h1>AC Assemble</h1>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        {acCards}
      </div>
    </div>
  );
}

