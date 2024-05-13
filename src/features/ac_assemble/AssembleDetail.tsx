import { useRouter } from "next/router";
import Image from "next/image";

// ダミーデータ
import { acAssembles } from "./ac6Types";



// オプションをインポートする
import { Autoplay, Navigation, Pagination } from "swiper/modules";
import { Swiper, SwiperSlide } from "swiper/react";
import "swiper/css";
import "swiper/css/navigation";
import "swiper/css/pagination";
import { AcAssemble, Frame, Weapons } from "../../../share/assemble_type";
import { useEffect, useState } from "react";

const ASSEMBLE_URL = "http://127.0.0.1:8000/ac?ulid=01HXPG5RS5C0H3ZBCMRTZVC0JN";

const defaultAcAssemble: AcAssemble = {
  ulid: "",
  pilotName: "Jhon Doe",
  acName: "",
  acCardImageUrl: "",
  emblemImageUrl: "",
  acImageUrls: [""],
  parts: {
    weapons: {
      rArm: "Rifle",
      lArm: "Shield",
      rBack: "Missile",
      lBack: "Rifle",
    },
    frame: {
      head: "Head",
      core: "Core",
      arms: "Arms",
      legs: "Legs",
    },
  },
  description: "",
  remarks: "",
};

interface AcAsmGetRes {
  ac_assemble: AcAssemble;
}

// const defaultAcAsmGetRes: AcAsmGetRes = {
//   acAssemble: defaultAcAssemble,
// };

export default function AssembleDetail() {
  // クエリパラメータからUUIDを取得 
  const router = useRouter();
  // const uuid = router.query.uuid;
  const ulid = "01HXPG5RS5C0H3ZBCMRTZVC0JN";
  
  // const acAsm = acAssembles[0];
  const [acAsm, setAcAsm] = useState<AcAssemble | undefined>(undefined);

  useEffect(() => {
    if (ulid) {
      fetch(`${ASSEMBLE_URL}`)
        .then(response => {
          console.log(response);
          return response.json()
        })
        .then(data => setAcAsm(data))
        .catch(error => console.error(error));
    }
  }, [ulid]);


  return (
    <div className="min-h-full w-screen flex flex-col justify-center items-center gap-5">
      <div className="flex border-4">
        <img className="w-32 h-32" src={acAsm?.emblemImageUrl} alt="Emblem" />
        <div className="m-5 ">
          <h1 className="text-2xl font-bold">AC: {acAsm?.acName}</h1>
          <h2 className="text-xl">PILOT: {acAsm?.pilotName}</h2>
          <h2 className="text-xl">UUID: {ulid}</h2>
        </div>
      </div>
      <ImageSwipe images={acAsm?.acImageUrls ?? defaultAcAssemble.acImageUrls} />
      <WeaponView weapons={acAsm?.parts.weapons ?? defaultAcAssemble.parts.weapons} />
      <FrameView frame={acAsm?.parts.frame ?? defaultAcAssemble.parts.frame} />
      <Description description={acAsm?.description ?? defaultAcAssemble.description} />
      <p>備考：{acAsm?.remarks}</p>
    </div>
  );
}

interface ImageSwipeProps {
  images: string[];
}

function ImageSwipe({images}: ImageSwipeProps) {
  return (
    <Swiper
      modules={[Navigation, Pagination, Autoplay]}
      slidesPerView={"auto"} // ハイドレーションエラー対策
      centeredSlides={true} // スライドを中央に配置
      loop={true} // スライドをループさせる
      speed={1000} // スライドが切り替わる時の速度
      autoplay={{
        delay: 8000,
        disableOnInteraction: false,
      }} // スライド表示時間
      navigation // ナビゲーション（左右の矢印）
      pagination={{
        clickable: true,
        type: "progressbar",
      }} // ページネーション, クリックで対象のスライドに切り替わる
      className="w-auto max-w-7xl h-fit min-h-80 mx-auto border-2"
    >
      {images.map((src: string, index: number) => (
        <SwiperSlide key={index} className="border-4 border-gray-600">
          <Image
            src={src}
            width={2000}
            height={2000}
            // このwidthとheight指定の数字に意味，理由はない
            alt="Slider Image"
            sizes=""
            className="object-contain w-full"
          />
        </SwiperSlide>
      ))}
    </Swiper>
  )
}


interface PartViewProps {
  partType: string;
  partName: string;
}

function PartView({ partType, partName }: PartViewProps) {
  return (
    <div className="flex mb-1">
      <div className="w-48 text-center border-2 bg-slate-200">
        <h2>{partType}</h2>
      </div>
      <div className="w-72 border-2">
        <h2>{partName}</h2>
      </div>
    </div>
  )
}


function WeaponView({ weapons }: { weapons: Weapons }) {
  return (
    <div>
      <PartView partType="R-ARM" partName={weapons.rArm} />
      <PartView partType="L-ARM" partName={weapons.lArm} />
      <PartView partType="R-BACK" partName={weapons.rBack} />
      <PartView partType="L-BACK" partName={weapons.lBack} />
    </div>
  )
}

function FrameView({ frame }: { frame: Frame }) {
  return (
    <div>
      <PartView partType="HEAD" partName={frame.head} />
      <PartView partType="CORE" partName={frame.core} />
      <PartView partType="ARMS" partName={frame.arms} />
      <PartView partType="LEGS" partName={frame.legs} />
    </div>
  )
}


interface DescriptionProps {
  description: string;
}

function Description({ description }: DescriptionProps) {
  return (
    <div className="w-3/4 h-fit border-4">
      <p className="w-full whitespace-pre-line">{description}</p>
    </div>
  )
}