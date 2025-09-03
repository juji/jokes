import { JokeManager } from './providers/joke-manager';
import * as fs from 'fs';
import * as path from 'path';

const NUMBER_OF_JOKES = 10;

async function fetchAndSaveJokes() {
  console.log(`🎭 Fetching ${NUMBER_OF_JOKES} random jokes from random providers...\n`);
  
  // Initialize the joke manager
  const jokeManager = new JokeManager();
  
  try {
    // Get random jokes from random providers
    const jokes = await jokeManager.getMultipleJokes(NUMBER_OF_JOKES);
    
    // Create result directory if it doesn't exist
    const resultDir = path.join(__dirname, 'result');
    if (!fs.existsSync(resultDir)) {
      fs.mkdirSync(resultDir, { recursive: true });
    }
    
    // Create timestamp for unique filename
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const filename = `jokes-${timestamp}.json`;
    const filepath = path.join(resultDir, filename);
    
    // Save jokes to file
    fs.writeFileSync(filepath, JSON.stringify(jokes, null, 2));
    
    console.log(`✅ Successfully fetched and saved ${jokes.length} jokes to: ${filepath}\n`);
    
    // Display the jokes
    jokes.forEach((joke, index) => {
      console.log(`${index + 1}. [${joke.provider}]`);
      if (joke.type === 'single') {
        console.log(`   ${joke.joke.content}`);
      } else {
        console.log(`   Setup: ${joke.joke.setup}`);
        console.log(`   Punchline: ${joke.joke.punchline}`);
      }
      if (joke.category) {
        console.log(`   Category: ${joke.category}`);
      }
      console.log();
    });
    
  } catch (error) {
    console.error('❌ Failed to fetch jokes:', error);
  }
}

// Run the demo
if (require.main === module) {
  fetchAndSaveJokes().catch(console.error);
}

export { fetchAndSaveJokes };
