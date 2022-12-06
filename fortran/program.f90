
program aoc22day5
    ! print *, 'Hello, World!'

    character (len = 35) :: a


    open(1, file = 'input.txt', status = 'old')
    do n = 0, 7, 1
        read(1, fmt = '(A)') a
        print *,  a(2:2), a(6:6), a(10:10), a(14:14), a(18:18), a(22:22), a(26:26), a(30:30), a(34:34)
    end do
end program aoc22day5