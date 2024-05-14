import Layout from "@/components/Layout";
import AssembleDetail from "@/features/ac_assemble/AssembleDetail";
import { Suspense } from "react";

export default function Home() {
  return (
    <>
      <Layout>
        <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
          <Suspense fallback={<div>Loading...</div>} >
            <AssembleDetail />
          </Suspense>
        </section>
      </Layout>
    </>
  );
}