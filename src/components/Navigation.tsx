import Link from 'next/link';
import React from 'react';

function Navigation() {
  return (
    <nav className="absolute z-10 top-0 w-screen h-4 flex flex-row ">
      <ul className="m-5 gap-5 flex flex-row">
        <li>
          <Link href="/">Home</Link>
        </li>
        <li>
          <Link href="/login">Log In</Link>
        </li>
        <li>
          <Link href="/register">Register</Link>
        </li>
        {/* <li>
          <Link href="/assemble">Assemble</Link>
        </li> */}
        <li>
          <Link href="/debug">Debug</Link>
        </li>
        <li>
          <Link href="/exampleindex">Example-index</Link>
        </li>
      </ul>
    </nav>
  );
}

export default Navigation;
