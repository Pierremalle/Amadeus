import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/home";
import MyComposition from "./pages/myComposition";
import { Provider } from './provider';

function App() {
    return (
        <Router>
            <Provider>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/my_compositions" element={<MyComposition />} />
                </Routes>
            </Provider>
        </Router>
    );
}

export default App;
