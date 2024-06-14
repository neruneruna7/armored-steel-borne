import Layout from "@/components/Layout";
import AuthedAssembleDetail from "@/features/ac_assemble/AuthedAssembleDetail";
import { useRouter } from "next/router";
import { Suspense } from "react";

export default function Home() {
  const router = useRouter();
  let id_str = router.query.id;
  let id = Number(id_str);

  return (
    <>
      <Layout>
        <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
          <Suspense fallback={<div>Loading...</div>} >
            <AuthedAssembleDetail id={id} />
          </Suspense>
        </section>
      </Layout>
    </>
  );
}