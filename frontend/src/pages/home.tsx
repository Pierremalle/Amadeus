import { NavBar } from "../components/Navbar.tsx";
import { Cards } from "../components/Cards.tsx";
import { instance } from "../env.tsx";
import { useEffect, useState } from "react";

async function requestApp() {
    const response = await instance.get('songs');
    return response.data;
}

export default function Home() {
    const [songs, setSongs] = useState([]);

    useEffect(() => {
        requestApp().then((data) => setSongs(data));
    }, []);

    const recentSongs = [...songs].sort((a, b) => a.timestamp > b.timestamp ? -1 : 1);

    return (
        <div className="min-h-screen bg-gray-50">
            <NavBar />
            <div className="container mx-auto px-4 py-6">
                <section className="mb-12">
                    <h1 className="text-2xl font-bold mb-6 border-b pb-2">Les incontournables</h1>

                    {songs.length === 0 ? (
                        <p className="text-gray-500 text-center py-8">Aucun morceau trouvé</p>
                    ) : (
                        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                            {songs.map((song) => (
                                <Cards key={song.id.id} song={song} />
                            ))}
                        </div>
                    )}
                </section>

                <section>
                    <h1 className="text-2xl font-bold mb-6 border-b pb-2">Les plus récentes</h1>

                    {recentSongs.length === 0 ? (
                        <p className="text-gray-500 text-center py-8">Aucun morceau récent</p>
                    ) : (
                        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                            {recentSongs.map((song) => (
                                <Cards key={song.id.id} song={song} />
                            ))}
                        </div>
                    )}
                </section>
            </div>
        </div>
    );
}