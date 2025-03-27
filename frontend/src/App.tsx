import { NavBar } from "./components/NavBar.tsx";
import { Cards } from "./components/Cards.tsx"

export default function App() {
    return (
        <>
            <NavBar/>
            <h1 className="text-2xl ml-2 font-diplay">Les incontournables</h1>
            <div className="grid grid-cols-4 gap-4">
                <Cards/>
                <Cards/>
                <Cards/>
                <Cards/>
                <Cards/>
                <Cards/>
            </div>
            <h1 className="text-2xl ml-2 mt-6">Les plus récentes</h1>
            <div className="grid grid-cols-4 gap-4">
                <Cards/>
                <Cards/>
                <Cards/>
                <Cards/>
                <Cards/>
                <Cards/>
            </div>
        </>
    )
}
