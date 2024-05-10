import Layout from "@/components/Layout";
import Image from "next/image";
import { useState } from "react";

interface AssembleData {
  name: string;
  imageUrls: string[];
  parts: string[];
  description: string;
  notes: string;
}

interface AsmCardProps {
  acPilotName: string,
  acName: string;
  acImageUrl: string;
  emblemImageUrl: string;
  description: string;
}

function AssembleCard({ mech }: { mech: AsmCardProps }) {
  return (
    <div className="border-4 w-80 h-72 relative">
      <div className="flex h-auto border-2">
        <div className="">
          <Image src={mech.emblemImageUrl} alt={`Mech Image`} width={70} height={70} className="object-contain border-2" />
        </div>
        <div className="ml-4">
          <h2>PILOT: {mech.acPilotName}</h2>
          <h2>AC: {mech.acName}</h2>
        </div>
        {/* <div className="border-4">
          <p>{mech.description}</p>
        </div> */}
      </div>

      <div className="border-2 relative w-full h-56">
        <Image src={mech.acImageUrl} alt={`Mech Image`} fill className="object-contain" />
      </div>
    </div>
  );
}

export default function AssembleBoard() {
  const [mechData, setMechData] = useState<AssembleData>({
    name: "Armored Steel Borne",
    imageUrls: [
      "https://example.com/image1.jpg",
      "https://example.com/image2.jpg",
      "https://example.com/image3.jpg",
    ],

    parts: ["Part 1", "Part 2", "Part 3"],
    description: "This is a description of the mech.",
    notes: "Some additional notes about the mech.",
  });

  const mech = {
    acPilotName: "John Doe",
    acName: "Armored Steel Borne",
    acImageUrl: "/ac/ac.jpg",
    emblemImageUrl: "/ac/rusty.jpg",
    description: "This is a description of the mech.",
  };

  return (
    <Layout>
      <div className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
        <h1>AC Assemble</h1>
        <AssembleCard mech={mech} />
      </div>
    </Layout>
  );
}