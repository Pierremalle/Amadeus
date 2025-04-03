import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/default";
import MyComposition from "./pages/my_composition";
import { Provider } from './provider';

function App() {
    return (
        <Router>
            <Provider>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/my_composition" element={<MyComposition />} />
                </Routes>
            </Provider>
        </Router>
    );
}

export default App;
