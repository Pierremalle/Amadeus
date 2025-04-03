import { NavBar } from "../components/Navbar.tsx";
import { Cards } from "../components/Cards.tsx"
import { instance } from "../env.tsx"
import { useEffect, useState } from "react";
import { SingleFileUploader } from "@/components/Button.tsx";

async function requestApp() {
    const response = await instance.get('songs');
    return response.data;
}

export default function Home() {
    const [songs, setSongs] = useState([]);
    useEffect(() => {
        requestApp().then((data) => setSongs(data));
    }, []);
    return (
        <>
            <NavBar/>
            <div className="grid grid-cols-4 gap-4">
                <SingleFileUploader/>
            </div>
            <h1 className="text-2xl ml-2 font-diplay">Les incontournables</h1>
            <div className="grid grid-cols-4 gap-4">
                {
                    songs.map((song) => <Cards key={song.id.id} song={song}/>)
                }
            </div>
            <h1 className="text-2xl ml-2 mt-6">Les plus récentes</h1>
            <div className="grid grid-cols-4 gap-4">
                {
                    songs.sort((a, b) => a.timestamp > b.timestamp ? -1 : 1).map((song) => <Cards key={song.id.id}
                                                                                                  song={song}/>)
                }
            </div>
        </>
    )
}