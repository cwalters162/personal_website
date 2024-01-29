export default function MenuBar() {
  return (
    <div
      className={
        "bg-light dark:bg-dark sticky top-0 z-50 flex w-full opacity-75"
      }
    >
      <ul className={"flex-2 flex"}>
        <li className={"text-primary-dark hidden p-4 text-2xl sm:block"}>
          Cherokee Walters
        </li>
        <li className={"hover:text-primary-dark hidden p-4 lg:block"}>
          <a href={"#experience"}>Experience</a>
        </li>
        <li className={"hover:text-primary-dark hidden p-4 lg:block"}>
          <a href={"#projects"}>Personal Projects</a>
        </li>
        <li className={"hover:text-primary-dark hidden p-4 lg:block"}>
          <a href={"#technologies"}>Technologies</a>
        </li>
      </ul>
      <ul className={"flex w-full flex-1 justify-center gap-4 sm:justify-end"}>
        <li className={"hover:text-primary-dark p-4"}>
          <a href={"https://github.com/cwalters162"} target={"_blank"}>
            GitHub
          </a>
        </li>
        <li className={"hover:text-primary-dark p-4"}>Linkedin</li>
        <li className={"hover:text-primary-dark p-4"}>Email</li>
      </ul>
    </div>
  )
}
