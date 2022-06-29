import type { NextPage } from "next";
import { SnakeGame } from "../components/SnakeGame";

const Home: NextPage = () => {
  return (
    <div className="flex items-center justify-center min-h-screen p-2 min-w-screen bg-gradient-to-br from-emerald-400 to-indigo-300">
      <SnakeGame />
    </div>
  );
};

export default Home;
