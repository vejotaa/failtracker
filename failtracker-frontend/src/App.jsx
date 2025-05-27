import { useEffect, useState } from 'react';
import { BarChart, Bar, XAxis, YAxis, Tooltip, CartesianGrid, ResponsiveContainer } from 'recharts';

const adjetivos = [
  'Rápido',
  'Brilhante',
  'Sorridente',
  'Travesso',
  'Zumbido',
  'Furioso',
  'Misterioso',
  'Sábio',
  'Esperto',
  'Saltitante',
];

const cores = [
  'Vermelho',
  'Azul',
  'Verde',
  'Amarelo',
  'Roxo',
  'Laranja',
  'Preto',
  'Branco',
  'Cinza',
  'Dourado',
];

const animais = [
  'Tigre',
  'Panda',
  'Canguru',
  'Coruja',
  'Foca',
  'Pinguim',
  'Lobo',
  'Raposa',
  'Gato',
  'Elefante',
];

function nomeAleatorio() {
  const adj = adjetivos[Math.floor(Math.random() * adjetivos.length)];
  const cor = cores[Math.floor(Math.random() * cores.length)];
  const animal = animais[Math.floor(Math.random() * animais.length)];
  return `${adj} ${cor} ${animal}`;
}

function App() {
  const [falhas, setFalhas] = useState([]);
  const [loading, setLoading] = useState(false);

  const API_URL = 'http://localhost:3000/falhas';

  useEffect(() => {
    fetchFalhas();
    const interval = setInterval(fetchFalhas, 5000);
    return () => clearInterval(interval);
  }, []);


  const limparFalhas = async () => {
    if (!window.confirm('Tem certeza que deseja apagar todas as falhas?')) return;

    try {
      setLoading(true);
      const res = await fetch(API_URL, { method: 'DELETE' });
      if (!res.ok) throw new Error('Erro ao limpar falhas');
      await fetchFalhas();
    } catch (err) {
      console.error(err);
      alert('Erro ao limpar falhas');
    } finally {
      setLoading(false);
    }
  };


  const fetchFalhas = async () => {
    try {
      const res = await fetch(API_URL);
      if (!res.ok) throw new Error('Erro ao buscar falhas');
      const data = await res.json();
      setFalhas(data);
    } catch (err) {
      console.error(err);
      alert('Erro ao buscar falhas');
    }
  };

  const registrarFalha = async () => {
    try {
      setLoading(true);
      const res = await fetch(API_URL, { method: 'POST' });
      if (!res.ok) throw new Error('Erro ao registrar falha');
      await fetchFalhas();
    } catch (err) {
      console.error(err);
      alert('Erro ao registrar falha');
    } finally {
      setLoading(false);
    }
  };

  // Função para mapear as falhas por dia da semana dos últimos 3 meses
  const getFalhasPorDiaSemana = () => {
    // filtro para últimos 3 meses
    const tresMesesAtras = new Date();
    tresMesesAtras.setMonth(tresMesesAtras.getMonth() - 3);

    const contagem = [0, 0, 0, 0, 0, 0, 0]; // Domingo = 0 ... Sábado = 6

    falhas.forEach(({ timestamp }) => {
      const data = new Date(timestamp);
      if (data >= tresMesesAtras) {
        contagem[data.getDay()] += 1;
      }
    });

    const dias = ['Dom', 'Seg', 'Ter', 'Qua', 'Qui', 'Sex', 'Sáb'];

    return dias.map((dia, i) => ({
      dia,
      falhas: contagem[i],
    }));
  };

  const falhasPorDia = getFalhasPorDiaSemana();

  return (
    <div style={{ display: 'flex', height: '100vh', padding: '1rem', fontFamily: 'Arial, sans-serif' }}>
      <div style={{ width: '300px', marginRight: '2rem' }}>
        <button onClick={registrarFalha} disabled={loading} style={{ padding: '0.5rem 1rem', marginBottom: '1rem', cursor: 'pointer' }}>
          {loading ? 'Registrando...' : 'Registrar nova falha'}
        </button>
        <button
          onClick={limparFalhas}
          disabled={loading}
          style={{
            padding: '0.5rem 1rem',
            marginBottom: '1rem',
            marginLeft: '0.5rem',
            cursor: 'pointer',
            backgroundColor: '#e74c3c',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            }}
          >
            {loading ? 'Processando...' : 'Limpar todas as falhas'}
        </button>

        <h2>Falhas Registradas</h2>
        <ul style={{ listStyle: 'none', paddingLeft: 0, maxHeight: '80vh', overflowY: 'auto', border: '1px solid #ccc', borderRadius: '5px' }}>
        {falhas.map((falha) => (
          <li key={falha.id} title={falha.id} style={{ padding: '0.5rem', borderBottom: '1px solid #eee' }}>
            <strong>{falha.nome}</strong> — {new Date(falha.timestamp).toLocaleString()}
          </li>
        ))}
        </ul>

      </div>
      <div style={{ flexGrow: 1 }}>
        <h2>Falhas por Dia da Semana (últimos 3 meses)</h2>
        <ResponsiveContainer width="100%" height={400}>
          <BarChart data={falhasPorDia} margin={{ top: 20, right: 30, left: 0, bottom: 20 }}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="dia" />
            <YAxis allowDecimals={false} />
            <Tooltip />
            <Bar dataKey="falhas" fill="#8884d8" />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

export default App;
