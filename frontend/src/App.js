import { useState } from "react";

function App() {
  const [wallet, setWallet] = useState("");
  const [data, setData] = useState(null);

  const trackWallet = async () => {
  try {
    const res = await fetch(
      `http://localhost:8000/wallet/${wallet}`
    );

    if (!res.ok) {
      throw new Error("Invalid wallet or server error");
    }

    const json = await res.json();
    setData(json);
  } catch (err) {
    alert("❌ Failed to fetch wallet data");
    console.error(err);
  }
};

  return (
    <div className="container">
      <h1>🐋 Solana Whale Tracker</h1>

      <input
        placeholder="Enter Solana Wallet Address"
        value={wallet}
        onChange={(e) => setWallet(e.target.value)}
      />

      <button onClick={trackWallet}>Track Wallet</button>

      {data && (
        <div className="result">
          <p>Balance: {data.balance.toFixed(2)} SOL</p>
          <p>SOL Price: ${data.price}</p>
          <p>Value: ${data.value_usd.toFixed(2)}</p>
        </div>
      )}
    </div>
  );
}

export default App;