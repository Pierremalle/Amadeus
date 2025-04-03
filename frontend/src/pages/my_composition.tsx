import { NavBar } from "./components/Navbar.tsx";
import { Cards } from "./components/Cards.tsx"
import { instance } from "./env.tsx"
import { useEffect, useState } from "react";
import { SingleFileUploader } from "@/components/Button.tsx";

async function requestApp() {
    const response = await instance.get('songs');
    return response.data;
}

export default function App() {
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
            <div className="grid grid-cols-4 gap-4">
                {
                    songs.map((song) => <Cards key={song.id.id} song={song}/>)
                }
            </div>
        </>
    )
}