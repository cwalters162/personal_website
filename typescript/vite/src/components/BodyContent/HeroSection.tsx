import Portrait from "../../assets/create-verity-logo-2.png"

export default function HeroSection() {
  return (
    <div>
      <div className={"flex"}>
        <div className={"pl-10"}>
          <h1 className={"pb-3 text-5xl"}>
            <span className={"dark:text-primary-dark"}>Best software </span>
            <span>you have ever seen</span>
          </h1>
          <div className={"flex justify-center pt-10"}>
            <p className={"w-2/3 text-center text-3xl"}>
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
            <button className={"rounded-full bg-primary-dark p-2"}>
              Contact Now
            </button>
          </div>
        </div>
        <div
          className={
            "justify-middle flex hidden overflow-clip rounded-full bg-red-400 sm:block"
          }
        >
          <img src={Portrait} alt={"headshot of Cherokee Walters"} />
        </div>
      </div>
    </div>
  )
}
