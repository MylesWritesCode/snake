import { useEffect } from "react";
import { useSnakeGame } from "../hooks/useSnakeGame";
import { greet } from "snake-game";

export function SnakeGame() {
  const { isLoaded } = useSnakeGame();

  useEffect(() => {
    if (!isLoaded) {
      return;
    }
  }, [isLoaded]);

  return (
    <div className="bg-slate-50 w-[500px] p-2 h-[500px]">
      <h2>works</h2>
    </div>
  );
}
