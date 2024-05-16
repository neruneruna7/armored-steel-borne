import Image from "next/image";

// オプションをインポートする
import { Autoplay, Navigation, Pagination } from "swiper/modules";
import { Swiper, SwiperSlide } from "swiper/react";
import "swiper/css";
import "swiper/css/navigation";
import "swiper/css/pagination";
import { AcAsmGetRes, AcAsmListRes, Frame, Weapons } from "../../../share/assemble_type";


const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
async function getAsm(ulid: string | string[] | undefined): Promise<AcAsmGetRes> {
  const ASSEMBLE_URL = "http://127.0.0.1:8000/asm/";
  // とりあえず動作確認のためにundifinedも許容
  // await sleep(1000);
  try {
    const res = await fetch(`${ASSEMBLE_URL}${ulid}`);
    console.log(res);
    const data = await res.json();
    console.log(data);
    return data;
  } catch (e: any) {
    console.log(`Error: ${e}`);
    return e;
  }
}

const dataMap: Map<string, AcAsmGetRes> = new Map();

function useData1(ulid: string): AcAsmGetRes {
  const cachedData = dataMap.get(ulid);
  if (cachedData === undefined) {
    throw getAsm(ulid).then((d) => dataMap.set(ulid, d));
  }
  return cachedData;
}

interface AssembleDetailProps {
  ulid: string
}

// let acAsmGetRes: AcAsmGetRes | undefined;
export default function AssembleDetail({ ulid }: AssembleDetailProps) {
  console.log("AssembleDetail");
  console.log(ulid);

  const acAsmGetRes = useData1(ulid);

  return (
    <div className="min-h-full w-screen flex flex-col justify-center items-center gap-5">
      <div className="flex border-4">
        <img className="w-32 h-32" src={acAsmGetRes.acAssemble.emblemImageUrl} alt="Emblem" />
        <div className="m-5 ">
          <h1 className="text-2xl font-bold">AC: {acAsmGetRes.acAssemble.acName}</h1>
          <h2 className="text-xl">PILOT: {acAsmGetRes.acAssemble.pilotName}</h2>
          <h2 className="text-xl">ULID: {ulid}</h2>
        </div>
      </div>
      <ImageSwipe images={acAsmGetRes.acAssemble.acImageUrls} />
      <WeaponView weapons={acAsmGetRes.acAssemble.parts.weapons} />
      <FrameView frame={acAsmGetRes.acAssemble.parts.frame} />
      <Description description={acAsmGetRes.acAssemble.description} />
      <p>備考：{acAsmGetRes.acAssemble.remarks}</p>
    </div>
  );
}

interface ImageSwipeProps {
  images: string[];
}

function ImageSwipe({ images }: ImageSwipeProps) {
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