import Layout from "@/components/Layout";
import AssembleBoard from "@/features/ac_assemble/AssembleBoard";

export default function Home() {
  return (
    <>
      <Layout>
        <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
          {/* <h1>AC Assemble</h1> */}
          <AssembleBoard />
        </section>
      </Layout>
    </>
  );
}