import { JokeProvider, Joke } from './types';

export class OfficialJokeProvider implements JokeProvider {
  name = 'Official Joke API';
  baseUrl = 'https://official-joke-api.appspot.com';
  
  private categories = [
    'general', 'programming', 'knock-knock', 'dad'
  ];

  async getRandomJoke(): Promise<Joke> {
    const response = await fetch(`${this.baseUrl}/random_joke`);
    const data = await response.json();
    
    return {
      id: data.id?.toString(),
      joke: {
        setup: data.setup,
        punchline: data.punchline
      },
      category: data.type?.toLowerCase(),
      type: 'twopart'
    };
  }

  async getJokeByCategory(category: string): Promise<Joke> {
    const validCategory = this.categories.includes(category.toLowerCase()) 
      ? category.toLowerCase() 
      : 'general';
    
    const response = await fetch(`${this.baseUrl}/jokes/${validCategory}/random`);
    const data = await response.json();
    
    // API returns an array, so take the first joke
    const joke = Array.isArray(data) ? data[0] : data;
    
    return {
      id: joke.id?.toString(),
      joke: {
        setup: joke.setup,
        punchline: joke.punchline
      },
      category: joke.type?.toLowerCase(),
      type: 'twopart'
    };
  }

  getSupportedCategories(): string[] {
    return this.categories;
  }
}
