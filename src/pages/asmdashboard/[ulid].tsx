import Layout from "@/components/Layout";
import AuthedAssembleDetail from "@/features/ac_assemble/AuthedAssembleDetail";
import { useRouter } from "next/router";
import { Suspense } from "react";

export default function Home() {
  const router = useRouter();

  let ulid = router.query.ulid;
  if (typeof ulid !== "string") {
    // ulidがstring型でない場合、空文字列を設定します。
    // このやり方が正しいのかはわからないけど，とりあえずこれで対処
    ulid = "";
  }

  return (
    <>
      <Layout>
        <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
          <Suspense fallback={<div>Loading...</div>} >
            <AuthedAssembleDetail ulid={ulid} />
          </Suspense>
        </section>
      </Layout>
    </>
  );
}