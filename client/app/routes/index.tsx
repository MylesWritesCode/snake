import { ClientOnly } from "remix-utils";
import { SnakeGame } from "~/components/SnakeGame.client";

export default function Index() {
  return (
    <div className="flex items-center justify-center min-h-screen p-2 min-w-screen bg-gradient-to-br from-emerald-400 to-indigo-300">
      <div className="bg-slate-50 w-[500px] p-2 h-[500px]">
        <ClientOnly fallback={<p>Loading...</p>}>
          {() => <SnakeGame />}
        </ClientOnly>
      </div>
    </div>
  );
}
