import { JokeProvider, Joke } from './types';
import { ALL_PROVIDERS } from './index';

export class JokeManager {
  private providers: JokeProvider[];

  constructor(providers: JokeProvider[] = ALL_PROVIDERS) {
    this.providers = providers;
  }

  /**
   * Get a random joke from a random provider
   */
  async getRandomJoke(): Promise<Joke & { provider: string }> {
    const randomProvider = this.providers[Math.floor(Math.random() * this.providers.length)];
    const joke = await randomProvider.getRandomJoke();
    
    return {
      ...joke,
      provider: randomProvider.baseUrl
    };
  }

  /**
   * Get a joke from a specific provider
   */
  async getJokeFromProvider(providerName: string): Promise<Joke & { provider: string }> {
    const provider = this.providers.find(p => 
      p.name.toLowerCase().includes(providerName.toLowerCase())
    );
    
    if (!provider) {
      throw new Error(`Provider "${providerName}" not found`);
    }
    
    const joke = await provider.getRandomJoke();
    return {
      ...joke,
      provider: provider.baseUrl
    };
  }

  /**
   * Get a joke by category from any provider that supports it
   */
  async getJokeByCategory(category: string): Promise<Joke & { provider: string }> {
    const providersWithCategory = this.providers.filter(p => 
      p.getSupportedCategories && 
      p.getSupportedCategories().some(cat => 
        cat.toLowerCase().includes(category.toLowerCase())
      )
    );

    if (providersWithCategory.length === 0) {
      // Fallback to random joke if no provider supports the category
      return this.getRandomJoke();
    }

    const randomProvider = providersWithCategory[Math.floor(Math.random() * providersWithCategory.length)];
    const joke = await randomProvider.getJokeByCategory!(category);
    
    return {
      ...joke,
      provider: randomProvider.baseUrl
    };
  }

  /**
   * Get jokes from multiple providers
   */
  async getMultipleJokes(count: number = 3): Promise<Array<Joke & { provider: string }>> {
    const jokes: Array<Promise<Joke & { provider: string }>> = [];

    for (let i = 0; i < count; i++) {
      try {
        jokes.push(this.getRandomJoke());
      } catch (error) {
        console.error(`Failed to get joke ${i + 1}:`, error);
      }
    }

    return Promise.all(jokes);
  }

  /**
   * List all available providers
   */
  getProviders(): Array<{ name: string; baseUrl: string; categories: string[] }> {
    return this.providers.map(provider => ({
      name: provider.name,
      baseUrl: provider.baseUrl,
      categories: provider.getSupportedCategories?.() || []
    }));
  }

  /**
   * Get all available categories across all providers
   */
  getAllCategories(): string[] {
    const categories = new Set<string>();
    
    this.providers.forEach(provider => {
      if (provider.getSupportedCategories) {
        provider.getSupportedCategories().forEach(cat => categories.add(cat));
      }
    });
    
    return Array.from(categories).sort();
  }
}
