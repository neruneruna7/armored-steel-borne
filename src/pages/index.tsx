import Layout from "@/components/Layout";
import AssembleBoard from "@/features/ac_assemble/AssembleBoard";
import { Suspense } from "react";

export default function Home() {
  return (
    <>
      <Layout>
        <Suspense fallback={<div>Loading...</div>} >
          <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
            <AssembleBoard />
          </section>
        </Suspense>
      </Layout>
    </>
  );
}