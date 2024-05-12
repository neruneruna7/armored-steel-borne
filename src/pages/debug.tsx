import Layout from "@/components/Layout";
import AssembleDetail from "@/features/ac_assemble/AssembleDetail";

export default function Home() {
  return (
    <>
      <Layout>
        <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
          <AssembleDetail />
        </section>
      </Layout>
    </>
  );
}