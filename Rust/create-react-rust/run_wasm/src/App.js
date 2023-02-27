import './App.css';
import { greet } from 'create-react-rust'

function App() {
  greet()
  return (
    <div className="App">
      <p>Hello wasm</p>
      <p></p>
    </div>
  );
}

export default App;
