type AcAsm = {
  pilotName: string;
  mechName: string;
  emblemUrl: string;
  mechImages: string[];
  partsList: string[];
  description: string;
  remarks: string;
};

// {
//   pilotName,
//   mechName,
//   emblemUrl,
//   mechImages,
//   partsList,
//   description,
//   remarks,
// }: MechDetailProps)

const acAsm = {
  pilotName: "John Doe",
  acName: "Armored Steel Borne",
  emblemUrl: "/ac/rusty.jpg",
  acImages: ["/ac/ac.jpg", "/ac/ac.jpg", "/ac/ac.jpg"],
  partsList: ["Part 1", "Part 2", "Part 3"],
  description: "This is a description",
  remarks: "These are remarks",
};

export default function AssembleDetail() {
  return (
    <div className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
      <h1 className="text-2xl font-bold">{acAsm.acName}</h1>
      <h2 className="text-xl">{acAsm.pilotName}</h2>
      <img className="w-32 h-32" src={acAsm.emblemUrl} alt="Emblem" />
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        {acAsm.acImages.map((image, index) => (
          <img key={index} src={image} alt={`Mech ${index + 1}`} />
        ))}
      </div>
      <ul>
        {acAsm.partsList.map((part, index) => (
          <li key={index}>{part}</li>
        ))}
      </ul>
      <p>{acAsm.description}</p>
      <p>{acAsm.remarks}</p>
    </div>
  );
}