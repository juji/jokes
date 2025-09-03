export interface Joke {
  id?: string;
  joke: {
    content?: string;    // For single-type jokes
    setup?: string;      // For twopart jokes
    punchline?: string;  // For twopart jokes
  };
  category?: string;
  type?: 'single' | 'twopart';
  safe?: boolean;
  lang?: string;
}

export interface JokeProvider {
  name: string;
  baseUrl: string;
  getRandomJoke(): Promise<Joke>;
  getJokeByCategory?(category: string): Promise<Joke>;
  getSupportedCategories?(): string[];
}

export interface JokeApiResponse {
  [key: string]: any;
}
