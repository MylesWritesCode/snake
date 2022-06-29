import type { NextPage } from "next";
import { useEffect } from "react";
import { useSnakeGame } from "../hooks/useSnakeGame";
import { greet } from "snake-game";

const Home: NextPage = () => {
  const { isLoaded } = useSnakeGame();

  useEffect(() => {
    if (!isLoaded) {
      return;
    }

    greet("Myles");
  }, [isLoaded]);

  return (
    <div className="flex items-center justify-center min-h-screen p-2 min-w-screen bg-gradient-to-br from-emerald-400 to-indigo-300">
      <div className="bg-slate-50 w-[500px] p-2 h-[500px]">
        <h1>works</h1>
      </div>
    </div>
  );
};

export default Home;
