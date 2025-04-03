import { NavBar } from "../components/Navbar.tsx";
import { Cards } from "../components/Cards.tsx";
import { instance } from "../env.tsx";
import { useEffect, useState } from "react";
import { SingleFileUploader } from "@/components/SingleFileUploader.tsx";

async function requestApp() {
    const response = await instance.get('songs');
    return response.data;
}

export default function MyComposition() {
    const [songs, setSongs] = useState([]);

    useEffect(() => {
        requestApp().then((data) => setSongs(data));
    }, []);

    return (
        <div className="min-h-screen bg-gray-50">
            <NavBar />
            <div className="container mx-auto px-4 py-6">
                <div className="max-w-3xl mx-auto">
                    <SingleFileUploader />
                </div>

                <div className="mt-8">
                    <h1 className="text-2xl font-bold mb-6 border-b pb-2">Mes compositions</h1>

                    {songs.length === 0 ? (
                        <p className="text-gray-500 text-center py-8">Aucune composition trouvée</p>
                    ) : (
                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                            {songs.map((song) => (
                                <Cards key={song.id.id} song={song} />
                            ))}
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}