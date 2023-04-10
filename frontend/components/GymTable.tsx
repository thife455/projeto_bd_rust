import { useRouter } from "next/router";
import React from "react";

function GymTable({ data }) {
  const router = useRouter();
  return (
    <>
      <table className="min-w-full divide-y divide-gray-200">
        <thead>
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Nome
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Endereco
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Cidade
            </th>
          </tr>
        </thead>
        <tbody>
          {data.map((gym) => (
            <tr key={gym.id}>
              <td className="px-6 py-4 whitespace-nowrap">{gym.name}</td>
              <td className="px-6 py-4 whitespace-nowrap">{gym.address}</td>
              <td className="px-6 py-4 whitespace-nowrap">{gym.city}</td>
              <td className="px-6 py-4 whitespace-nowrap">
                <button
                  onClick={() => {
                    router.push(`gyms/${gym.id}`);
                  }}
                  className="bg-blue-500 text-white rounded-md"
                >
                  <p className="m-3">Detalhes</p>
                </button>
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <button
                  onClick={() => {
                    router.push(`gyms/${gym.id}/users`);
                  }}
                  className="bg-blue-500 text-white rounded-md"
                >
                  <p className="m-3">Usuarios</p>
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}

export default GymTable;
