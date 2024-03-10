// import Portrait from "../../assets/create-verity-logo-2.png"

export default function HeroSection() {
  return (
    <div>
      <div className={"flex justify-center"}>
        <div>
          <div className={"pb-10 text-center text-3xl sm:text-5xl"}>
            <span>Empowering </span>
            <span className={"dark:text-primary-dark"}>Your Goals </span>
            <span>With </span>
            <span className={"dark:text-primary-dark"}>Great </span>
            <span>Software</span>
          </div>
          <div className={"flex justify-center"}>
            <p className={"w-2/3 text-center text-xl sm:text-3xl"}>
              <span>Experienced </span>
              software engineer using
              <span className={"dark:text-primary-dark"}> best practices </span>
              and
              <span className={"dark:text-primary-dark"}> scalable </span>
              technologies to
              <span className={"dark:text-primary-dark"}> exceed </span>
              your needs
            </p>
          </div>
          <div className={"pt-10 text-center"}>
            <button className="transform rounded-full border-2 border-primary-dark p-2 text-3xl transition duration-500 ease-in-out hover:-translate-y-1 hover:scale-110 dark:text-primary-dark">
              <a
                href={"https://www.linkedin.com/in/cwalters162/"}
                target={"_blank"}
              >
                Connect With Me
              </a>
            </button>
          </div>
        </div>
        <div
          className={
            "justify-middle hidden overflow-clip rounded-full bg-red-400 md:flex"
          }
        >
          {/*<img src={Portrait} alt={"headshot of Cherokee Walters"} />*/}
        </div>
      </div>
    </div>
  )
}
