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
    const [isUploaded, setIsUploaded] = useState(false);

    useEffect(() => {
        requestApp().then((data) => setSongs(data));
    }, [isUploaded]);

    const handleUploadSuccess = () => {
        setIsUploaded(prev => !prev);
    };

    return (
        <div className="min-h-screen bg-gray-50">
            <NavBar />
            <div className="container mx-auto px-4 py-6">
                <div className="max-w-3xl mx-auto mb-12">
                    <h1 className="text-2xl font-bold mb-6 border-b pb-2">Ajouter une composition</h1>
                    <SingleFileUploader onUploadSuccess={handleUploadSuccess} />
                </div>

                <section>
                    <h1 className="text-2xl font-bold mb-6 border-b pb-2">Mes compositions</h1>

                    {songs.length === 0 ? (
                        <div className="bg-white p-8 rounded-lg shadow-sm text-center">
                            <p className="text-gray-500 mb-2">Vous n'avez pas encore de compositions</p>
                            <p className="text-sm text-gray-400">Utilisez le formulaire ci-dessus pour ajouter votre première composition</p>
                        </div>
                    ) : (
                        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 justify-items-center">
                            {songs.map((song) => (
                                <Cards key={song.id.id} song={song} />
                            ))}
                        </div>
                    )}
                </section>
            </div>
        </div>
    );
}