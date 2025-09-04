
const prefix = process.env.NEXT_PUBLIC_API_URL;

export async function getRandomJoke() {
    const response = await fetch(`${prefix}/jokes/random`);
    if (!response.ok) {
        throw new Error("Failed to fetch random joke");
    }
    return response.json();
}

export async function saveRandomJoke(){
    const response = await fetch(`${prefix}/jokes/retrieve`);
    if (!response.ok) {
        throw new Error("Failed to retrieve random joke");
    }
    return response.json();
}