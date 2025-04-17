import { PhotoManagerWidget } from '@/widgets/photo-manager';

function IndexPage() {
  return (
    <main className="min-h-screen bg-[hsl(var(--background))]">
      <div className="container mx-auto py-10">
        <h1 className="text-4xl font-bold mb-8 text-center">포토 매니저</h1>
        <PhotoManagerWidget />
      </div>
    </main>
  );
}

export default IndexPage;
