import PlusIcon from "../assets/plus.svg"

export default function FloatingActionButton() {
  return (
    <button
      className={
        "fixed bottom-5 right-5 cursor-pointer rounded-full border-none bg-blue-500 px-5 py-2.5 text-white"
      }
    >
      <img src={PlusIcon} className={"h-5 w-5"} alt="Add" />
    </button>
  )
}
