import System.Info (os)
import System.IO (stdout, hFlush)
import Control.Monad (forever)
import System.Process (callCommand)
import Control.Concurrent (threadDelay)
import System.Exit (exitSuccess)

main :: IO ()
main = do
    let (shutdown, cancel, multiply) = 
            if os == "linux" then
                ("shutdown -h +", "shutdown -c", 1)
            else if os == "mingw32" || os == "cygwin" || os == "windows" then
                ("shutdown /s /t ", "shutdown /a", 60)
            else
                ("", "", 0)

    forever $ do
        callCommand "cls||clear"
        putStr"1- Set the shutdown time\n2- Cancel the shutdown\n3- Exit\n\n>>>"
        hFlush stdout
        menu <- getLine
        callCommand "cls||clear"
        if menu == "1" then do
            putStr "How many minutes later do you want the computer to shutdown: "
            hFlush stdout
            timeStr <- getLine
            let time_ = reads timeStr :: [(Int, String)]
            case time_ of
                [(timeInt, "")] -> do 
                    
                    callCommand("cls||clear && "++shutdown++show(timeInt*multiply))
                    threadDelay 3000000
                _ -> do
                    putStrLn "\nPlease type number.."
                    threadDelay 2000000
        else if menu == "2" then do
            callCommand cancel
            putStrLn "The shutdown has been canceled.."
            threadDelay 2000000
        else if menu == "3" then do
            exitSuccess
        else do
            putStrLn "Please make a valid choice.."
            threadDelay 2000000