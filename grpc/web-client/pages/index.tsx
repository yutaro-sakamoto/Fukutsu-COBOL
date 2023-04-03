import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import type { PartialMessage } from "@bufbuild/protobuf";
import styles from '@/styles/Home.module.css'
import { UserService } from "../services/fcbl-core_connectweb";
import type { NewCore, Core } from '../services/fcbl-core_pb';
import { createGrpcWebTransport, createPromiseClient } from '@bufbuild/connect-web';
import { FormEvent, useState } from 'react';

import { useMemo } from "react";
import { ServiceType } from "@bufbuild/protobuf";
import {
  createConnectTransport,
  PromiseClient,
  Transport,
} from "@bufbuild/connect-web";

/*const transport = createGrpcWebTransport({
  baseUrl: "http://127.0.0.1:50051",
});*/

function useClient<T extends ServiceType>(service: T): PromiseClient<T> {
  return createPromiseClient(service, createGrpcWebTransport({
    baseUrl: "http://127.0.0.1:50051",
  }));
}
const client = useClient(UserService);

export default function Home() {
  const [name, setName] = useState("");

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    const newCore: PartialMessage<NewCore> = { name: "hello" };
    await client.new_core(newCore).then((core) => {
      console.log(core.id);
    });
  };
  return (
    <>
      <form onSubmit={handleSubmit}>
        <input
          placeholder="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <button type="submit">submit</button>
      </form>
    </>
  );
}
