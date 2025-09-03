import { JokeProvider, Joke } from './types';

export class JokesOneProvider implements JokeProvider {
  name = 'Jokes One API';
  baseUrl = 'https://api.jokes.one';
  private apiKey?: string;

  constructor(apiKey?: string) {
    this.apiKey = apiKey;
  }

  async getRandomJoke(): Promise<Joke> {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json'
    };
    
    if (this.apiKey) {
      headers['X-JokesOne-Api-Secret'] = this.apiKey;
    }

    try {
      // Try to get a random joke from their API
      const response = await fetch(`${this.baseUrl}/jod`, { headers });
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const data = await response.json();
      
      return {
        id: data.joke?.id,
        joke: {
          content: data.joke?.text || data.contents?.jokes?.[0]?.joke?.text
        },
        category: data.joke?.category || 'general',
        type: 'single'
      };
    } catch (error) {
      // Fallback joke if API is not accessible
      return {
        joke: {
          content: "Why don't scientists trust atoms? Because they make up everything!"
        },
        category: 'science',
        type: 'single'
      };
    }
  }

  async getJokeByCategory(category: string): Promise<Joke> {
    // Jokes One API has limited category support without premium
    return this.getRandomJoke();
  }

  getSupportedCategories(): string[] {
    return ['general', 'dad', 'programming', 'science'];
  }
}
