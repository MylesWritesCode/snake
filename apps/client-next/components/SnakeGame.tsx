import { useEffect } from "react";
import { useSnakeGame } from "../hooks/useSnakeGame";
import { init } from "snake-game";

export function SnakeGame() {
  const { isLoaded } = useSnakeGame();

  useEffect(() => {
    if (!isLoaded) {
      return;
    }

    init.main();
  }, [isLoaded]);

  return (
    <div
      id="snakeGameContainer"
      className="bg-slate-50 w-[500px] p-2 h-[500px]">
      <h2>works</h2>
    </div>
  );
}
