import { FaGithub, FaLinkedin } from "react-icons/fa6"

export default function MenuBar() {
  return (
    <div
      className={
        "sticky top-0 z-50 flex w-full bg-light bg-opacity-75 py-3 dark:bg-dark dark:bg-opacity-75"
      }
    >
      <ul className={"flex"}>
        <li className={"hidden p-4 text-5xl text-yellow-400 sm:block"}>
          Cherokee Walters
        </li>
        <li
          className={
            "hidden p-4 hover:text-yellow-400 md:flex md:items-center md:text-2xl lg:text-xl"
          }
        >
          <a href={"#experience"}>Experience</a>
        </li>
        <li
          className={
            "hidden p-4 hover:text-yellow-400 md:items-center md:text-2xl lg:flex lg:text-xl"
          }
        >
          <a href={"#projects"}>Personal Projects</a>
        </li>
        <li
          className={
            "hidden p-4 hover:text-yellow-400 md:items-center md:text-2xl lg:flex lg:text-xl"
          }
        >
          <a href={"#technologies"}>Technologies</a>
        </li>
      </ul>
      <ul
        className={"flex flex-1 justify-around sm:justify-end sm:gap-8 sm:pr-4"}
      >
        <li className={"flex items-center"}>
          <a href={"https://github.com/cwalters162"} target={"_blank"}>
            <FaGithub className={"text-6xl hover:text-yellow-400"} />
          </a>
        </li>
        <li className={"flex items-center"}>
          <a href={""} target={"_blank"}>
            <FaLinkedin className={"text-6xl hover:text-yellow-400"} />
          </a>
        </li>
      </ul>
    </div>
  )
}
