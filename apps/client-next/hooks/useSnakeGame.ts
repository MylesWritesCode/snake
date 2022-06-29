import { useEffect, useState } from "react";
import wasm, { InitOutput } from "snake-game/pkg/snake_wasm";

export function useSnakeGame() {
  const [isLoaded, setIsLoaded] = useState(false);
  const [game, setGame] = useState<InitOutput>();

  useEffect(() => {
    run();
  }, []);

  async function run() {
    const res = await wasm().then((module) => {
      setIsLoaded(true);
      setGame(module);
    });
  }

  return { isLoaded };
}
