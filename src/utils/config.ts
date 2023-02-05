export interface Config {
  libraryPath: string;
}

export function createDefaultConfig(): Config {
  return {
    libraryPath: "",
  };
}
