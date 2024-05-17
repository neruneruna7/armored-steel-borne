import Layout from '../../components/Layout';
import AuthedAssembleBoard from '@/features/ac_assemble/AuthedAssembleBoard';
import { Suspense } from "react";


export default function Dashboard() {

  return (
    <Layout>
      <div className="flex flex-col gap-8 pt-10 justify-center bg-slate-50 items-center pb-10 overflow-y-scroll">
        <Suspense fallback={<div>Loading...</div>} >
          <AuthedAssembleBoard />
        </Suspense>      </div>
    </Layout>
  );
}


// export default function Home() {
//     return (
//         <>
//             <Layout>
//                 <Suspense fallback={<div>Loading...</div>} >
//                     <section className="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
//                         <AssembleBoard />
//                     </section>
//                 </Suspense>
//             </Layout>
//         </>
//     );
// }