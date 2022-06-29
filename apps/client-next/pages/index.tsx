import type { NextPage } from "next";
import { useEffect, useLayoutEffect, useState } from "react";

const Home: NextPage = () => {
  const [game, setState] = useState<any>();

  loadWasm();

  async function loadWasm() {
    await import("snake-game").then((module) => {
      setState(module);
    });
  }

  return (
    <div className="flex items-center justify-center min-h-screen p-2 min-w-screen bg-gradient-to-br from-emerald-400 to-indigo-300">
      <div className="bg-slate-50 w-[500px] p-2 h-[500px]">
        <h1>works</h1>
      </div>
    </div>
  );
};

export default Home;
