import Layout from '../../components/Layout';

export default function Upload() {

  return (
    <Layout>
      <div className="flex flex-col gap-8 pt-10 justify-center bg-slate-50 items-center pb-10 overflow-y-scroll">
        <button className='border-4  w-80 h-60'>upload</button>
      </div>
    </Layout>
  );
}
