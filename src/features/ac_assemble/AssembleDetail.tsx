import { useState } from "react";

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
  acImages: ["/ac/ac.jpg", "/ac/ac2.jpg", "/ac/ac3.png"],
  partsList: ["Part 1", "Part 2", "Part 3"],
  description: "This is a description",
  remarks: "These are remarks",
};

export default function AssembleDetail() {

  const [currentImageIndex, setCurrentImageIndex] = useState(0);

  const handlePrevImage = () => {
    setCurrentImageIndex((prevIndex) =>
      prevIndex === 0 ? acAsm.acImages.length - 1 : prevIndex - 1
    );
  };

  const handleNextImage = () => {
    setCurrentImageIndex((prevIndex) =>
      prevIndex === acAsm.acImages.length - 1 ? 0 : prevIndex + 1
    );
  };

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
      <div>
        <button onClick={handlePrevImage}>Prev</button>
        <img
          className="w-80 object-contain"
          src={acAsm.acImages[currentImageIndex]}
          alt={`Mech ${currentImageIndex + 1}`}
        />
        <button onClick={handleNextImage}>Next</button>
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




// export default function AssembleDetail() {

//   return (
//     <div className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
//       {/* ...existing code... */}
//       <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
//         <button onClick={handlePrevImage}>Prev</button>
//         <img
//           className="w-32 h-32"
//           src={acAsm.acImages[currentImageIndex]}
//           alt={`Mech ${currentImageIndex + 1}`}
//         />
//         <button onClick={handleNextImage}>Next</button>
//       </div>
//       {/* ...existing code... */}
//     </div>
//   );
// }